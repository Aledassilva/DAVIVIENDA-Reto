<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Crear Usuarios</name>
   <tag></tag>
   <elementGuidId>a0b05d3e-03f9-4514-913a-76abb242234f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;firstName\&quot;: \&quot;${nombre1}\&quot;,\n  \&quot;lastName\&quot;: \&quot;${nombre2}\&quot;,\n  \&quot;middleName\&quot;: \&quot;${apellido}\&quot;,\n  \&quot;employeeId\&quot;: \&quot;${codigo}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ddcf5058-84e4-41dd-96c7-83d67f5f92be</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>b360995c-33bf-493f-8165-23ca23bf98cc</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>orangehrm=cfcm49p1k61grsvb9rk66bhfvv; orangehrm=50obb6pdr6g107r53ij22rn1ns</value>
      <webElementGuid>33a5bbbd-6d1b-49da-94a5-359d1e1f4734</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://opensource-demo.orangehrmlive.com/web/index.php/api/v2/pim/employees</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'545'</defaultValue>
      <description></description>
      <id>1c60e711-0606-4d07-964c-367a0eab0542</id>
      <masked>false</masked>
      <name>codigo</name>
   </variables>
   <variables>
      <defaultValue>'Alejandro'</defaultValue>
      <description></description>
      <id>4aaf289b-80bc-46b6-ab90-f5039d7816f0</id>
      <masked>false</masked>
      <name>nombre1</name>
   </variables>
   <variables>
      <defaultValue>'Tinjaca'</defaultValue>
      <description></description>
      <id>20e49400-44e6-4dbc-810b-b9417ecc4249</id>
      <masked>false</masked>
      <name>nombre2</name>
   </variables>
   <variables>
      <defaultValue>'Davivienda'</defaultValue>
      <description></description>
      <id>3d285825-d7cd-4ea3-a812-d16db0618ec9</id>
      <masked>false</masked>
      <name>apellido</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
